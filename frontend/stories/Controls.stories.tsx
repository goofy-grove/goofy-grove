import { IconPencil, IconTrash } from '@tabler/icons-react';
import { useState } from 'react';

import { Button, Card, Dropdown, Input } from '@shared/ui';

import type { Meta, StoryObj } from '@storybook/react-vite';

const meta = { title: 'Controls/Overview' } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

export const Buttons: Story = {
  render: () => (
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12 }}>
      <Button>Сохранить</Button>

      <Button variant="ghost">Отмена</Button>

      <Button color="error" variant="ghost">
        Удалить
      </Button>

      <Button disabled>Сохраняем…</Button>
    </div>
  ),
};

export const Fields: Story = {
  render: function Fields() {
    const [value, setValue] = useState('Мох');

    return (
      <div style={{ maxWidth: 480, display: 'grid', gap: 24 }}>
        <Input label="Имя" value={value} onChange={setValue} />

        <Input label="Описание" multiline hint="Расскажите о своём герое." />

        <Input label="Недоступное поле" disabled value="Сохраняем…" />
      </div>
    );
  },
};

export const Panel: Story = {
  render: () => (
    <Card title="Новая история" style={{ maxWidth: 420 }}>
      <p style={{ padding: '0 24px 24px', margin: 0 }}>
        Место для героев и маленьких приключений.
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
        trigger={<Button variant="ghost">Действия</Button>}
      >
        <Button
          variant="ghost"
          leftIcon={<IconPencil size={18} />}
          onClick={() => setOpen(false)}
        >
          Редактировать
        </Button>

        <hr className="dropdown__separator" />

        <Button
          variant="ghost"
          color="error"
          leftIcon={<IconTrash size={18} />}
          onClick={() => setOpen(false)}
        >
          Удалить
        </Button>
      </Dropdown>
    );
  },
};
