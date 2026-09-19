import { IconArrowDown, IconLeaf, IconRefresh } from '@tabler/icons-react';
import { defaultRangeExtractor, useVirtualizer } from '@tanstack/react-virtual';
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from 'react';
import { useTranslation } from 'react-i18next';

import { CHAT_PAGE_SIZE } from '@pages/chats/model';
import { ChatMessage } from '@pages/chats/ui/chat-message';

import { Button } from '@shared/ui';

import type { VirtualTimelineProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const VirtualTimeline: FC<VirtualTimelineProps> = ({
  messages,
  hasOlder = false,
  loadOlder,
  pageSize = CHAT_PAGE_SIZE,
  onEdit,
  onRetry,
  footer,
  empty,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const scroller = useRef<HTMLDivElement>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [atBottom, setAtBottom] = useState(true);
  const [editingIds, setEditingIds] = useState<string[]>([]);
  const fetching = useRef(false);
  const pinned = useRef(true);
  const initialized = useRef(false);
  const mounted = useRef(true);
  const getItemKey = useCallback(
    (index: number) => messages[index].id,
    [messages],
  );
  const virtualizer = useVirtualizer({
    count: messages.length,
    getScrollElement: () => scroller.current,
    estimateSize: () => 230,
    getItemKey,
    overscan: 4,
    rangeExtractor: (range) =>
      [
        ...new Set([
          ...defaultRangeExtractor(range),
          ...editingIds
            .map((id) => messages.findIndex((message) => message.id === id))
            .filter((index) => index >= 0),
        ]),
      ].sort((a, b) => a - b),
    paddingStart: 56,
    anchorTo: 'end',
    followOnAppend: true,
    scrollEndThreshold: 64,
  });
  const virtualItems = virtualizer.getVirtualItems();

  useEffect(() => {
    mounted.current = true;

    return () => {
      mounted.current = false;
    };
  }, []);

  const requestOlder = async () => {
    if (fetching.current || !hasOlder || !loadOlder) return;
    pinned.current = false;
    fetching.current = true;
    setLoading(true);
    setError(false);
    try {
      await loadOlder(pageSize);
    } catch {
      if (mounted.current) setError(true);
    } finally {
      fetching.current = false;
      if (mounted.current) setLoading(false);
    }
  };

  useLayoutEffect(() => {
    // TanStack preserves the visible keyed row across prepends and measurements.
    if (!initialized.current && messages.length) {
      initialized.current = true;
      virtualizer.scrollToEnd();
    }
  }, [messages.length, virtualizer]);

  useEffect(() => {
    const content = scroller.current?.firstElementChild;

    if (!content) return;
    const observer = new ResizeObserver(() => {
      if (pinned.current && scroller.current)
        scroller.current.scrollTop = scroller.current.scrollHeight;
    });

    observer.observe(content);

    return () => observer.disconnect();
  }, []);

  const onScroll = () => {
    const element = scroller.current;

    if (!element) return;
    const bottom =
      element.scrollHeight - element.scrollTop - element.clientHeight < 64;

    pinned.current = bottom;
    setAtBottom(bottom);
    if (element.scrollTop < 120 && !error && !loading) void requestOlder();
  };

  return (
    <div className="chat-timeline-shell">
      <div
        ref={scroller}
        className="chat-timeline scrollbar"
        role="region"
        aria-label={t('loadOlder')}
        tabIndex={0}
        onScroll={onScroll}
      >
        <div className="chat-virtual-content">
          <div
            className="chat-virtual-spacer"
            style={{ height: virtualizer.getTotalSize() }}
          >
            <div className="chat-history-status">
              {loading ? (
                <span role="status" className="chat-history-loader">
                  <IconLeaf size={18} />

                  {t('loading')}
                </span>
              ) : error ? (
                <span role="alert">
                  {t('historyError')}

                  <Button
                    size="compact"
                    variant="ghost"
                    type="button"
                    onClick={() => void requestOlder()}
                  >
                    <IconRefresh size={16} />

                    {t('retry')}
                  </Button>
                </span>
              ) : hasOlder ? (
                <Button
                  size="compact"
                  variant="ghost"
                  type="button"
                  onClick={() => void requestOlder()}
                >
                  {t('loadOlder')}
                </Button>
              ) : (
                <div className="chat-scene-divider">
                  <IconLeaf size={15} />

                  <span>{t('scene')}</span>
                </div>
              )}
            </div>

            {virtualItems.map((row) => (
              <div
                key={row.key}
                ref={virtualizer.measureElement}
                data-index={row.index}
                data-message-id={messages[row.index].id}
                className="chat-virtual-row"
                style={{ transform: `translateY(${row.start}px)` }}
              >
                <ChatMessage
                  message={messages[row.index]}
                  onEdit={onEdit}
                  onRetry={onRetry}
                  onEditingChange={(id, editing) =>
                    setEditingIds((current) =>
                      editing
                        ? [...new Set([...current, id])]
                        : current.filter((value) => value !== id),
                    )
                  }
                />
              </div>
            ))}
          </div>

          {messages.length === 0 && empty}

          {footer}
        </div>
      </div>

      {!atBottom && (
        <Button
          size="compact"
          variant="ghost"
          type="button"
          className="chat-jump"
          onClick={() => {
            pinned.current = true;
            virtualizer.scrollToEnd();
            scroller.current?.scrollTo({ top: scroller.current.scrollHeight });
            setAtBottom(true);
          }}
        >
          <IconArrowDown size={16} />

          {t('latest')}
        </Button>
      )}
    </div>
  );
};
