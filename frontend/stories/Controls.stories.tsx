import { IconPencil, IconTrash } from '@tabler/icons-react';
import { useState } from 'react';

import { Button, Card, Dropdown, Input } from '@shared/ui';

import type { VirtualElement } from '@floating-ui/react';
import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = { title: 'Controls/Overview' } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

export const Buttons: Story = {
  render: () => (
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12 }}>
      <Button>Save</Button>

      <Button variant="ghost">Cancel</Button>

      <Button color="error" variant="ghost">
        Delete
      </Button>

      <Button disabled>Saving…</Button>
    </div>
  ),
};

export const Fields: Story = {
  render: function Fields() {
    const [value, setValue] = useState('Moss');

    return (
      <div style={{ maxWidth: 480, display: 'grid', gap: 24 }}>
        <Input label="Name" value={value} onChange={setValue} />

        <Input
          label="Description"
          multiline
          hint="Tell us about your character."
        />

        <Input label="Disabled field" disabled value="Saving…" />
      </div>
    );
  },
};

export const Panel: Story = {
  render: () => (
    <Card title="New story" style={{ maxWidth: 420 }}>
      <p style={{ padding: '0 24px 24px', margin: 0 }}>
        A place for characters and little adventures.
      </p>
    </Card>
  ),
};

export const ActionMenu: Story = {
  render: function ActionMenu() {
    const [open, setOpen] = useState(false);

    return (
      <Dropdown
        isOpen={open}
        onShow={() => setOpen(true)}
        onHide={() => setOpen(false)}
        trigger={<Button variant="ghost">Actions</Button>}
      >
        <Button
          variant="ghost"
          leftIcon={<IconPencil size={18} />}
          onClick={() => setOpen(false)}
        >
          Edit
        </Button>

        <hr className="dropdown__separator" />

        <Button
          variant="ghost"
          color="error"
          leftIcon={<IconTrash size={18} />}
          onClick={() => setOpen(false)}
        >
          Delete
        </Button>
      </Dropdown>
    );
  },
};

export const VirtualActionMenu: Story = {
  render: function VirtualActionMenu() {
    const [open, setOpen] = useState(false);
    const [anchor, setAnchor] = useState<VirtualElement | null>(null);

    return (
      <>
        <div
          style={{ padding: 64, border: '1px dashed var(--border-default)' }}
          onContextMenu={(event) => {
            event.preventDefault();

            const { clientX, clientY } = event;

            setAnchor({
              getBoundingClientRect: () => new DOMRect(clientX, clientY, 0, 0),
            });
            setOpen(true);
          }}
        >
          Right-click to open the menu.
        </div>

        <Dropdown
          isOpen={open}
          virtualReference={anchor}
          onHide={() => setOpen(false)}
        >
          <Button
            variant="ghost"
            leftIcon={<IconPencil size={18} />}
            onClick={() => setOpen(false)}
          >
            Edit
          </Button>

          <hr className="dropdown__separator" />

          <Button
            variant="ghost"
            color="error"
            leftIcon={<IconTrash size={18} />}
            onClick={() => setOpen(false)}
          >
            Delete
          </Button>
        </Dropdown>
      </>
    );
  },
};

export const SharedListActionMenu: Story = {
  render: function SharedListActionMenu() {
    const [activeItem, setActiveItem] = useState<{
      name: string;
      anchor: HTMLButtonElement;
    } | null>(null);

    return (
      <>
        <div style={{ display: 'grid', gap: 12, maxWidth: 320 }}>
          {['Moss', 'Fern', 'Willow'].map((name) => (
            <Button
              key={name}
              onClick={(event) => {
                event.stopPropagation();
                setActiveItem({ name, anchor: event.currentTarget });
              }}
            >
              Actions for {name}
            </Button>
          ))}
        </div>

        <Dropdown
          isOpen={activeItem !== null}
          virtualReference={activeItem?.anchor}
          onHide={() => setActiveItem(null)}
        >
          <Button
            variant="ghost"
            leftIcon={<IconPencil size={18} />}
            onClick={() => setActiveItem(null)}
          >
            Edit {activeItem?.name}
          </Button>

          <hr className="dropdown__separator" />

          <Button
            variant="ghost"
            color="error"
            leftIcon={<IconTrash size={18} />}
            onClick={() => setActiveItem(null)}
          >
            Delete {activeItem?.name}
          </Button>
        </Dropdown>
      </>
    );
  },
};
