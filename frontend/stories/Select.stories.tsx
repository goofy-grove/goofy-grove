import { useState } from 'react';
import { expect, userEvent, within } from 'storybook/test';

import { Select } from '@shared/ui/select';

import type { SelectItem } from '@shared/ui/select/types';
import type { Meta, StoryObj } from '@storybook/react-vite';

function SelectPreview({
  items,
  placeholder,
  multiple = false,
  initial = ['ru'],
}: {
  items: SelectItem[];
  placeholder: string;
  multiple?: boolean;
  initial?: string[];
}) {
  const [selected, setSelected] = useState(initial);

  return multiple ? (
    <Select
      items={items}
      placeholder={placeholder}
      multiselect
      selected={selected}
      onChange={setSelected}
    />
  ) : (
    <Select
      items={items}
      placeholder={placeholder}
      selected={selected[0]}
      onChange={(value) => setSelected([value])}
    />
  );
}

const meta = {
  title: 'Controls/Select',
  component: SelectPreview,
  args: {
    items: [
      { value: 'ru', label: 'Русский' },
      { value: 'en', label: 'English' },
      {
        value: 'long',
        label: 'Очень длинное название варианта для проверки переноса текста',
      },
    ],
    placeholder: 'Язык истории',
  },
  decorators: [
    (Story) => (
      <div style={{ width: 'min(100%, 320px)' }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof SelectPreview>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Single: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);

    await userEvent.click(canvas.getByRole('button', { name: 'Язык истории' }));
    await userEvent.click(await page.findByRole('option', { name: 'English' }));
    await expect(
      canvas.getByRole('button', { name: 'Язык истории' }),
    ).toHaveTextContent('English');
    await expect(page.queryByRole('listbox')).not.toBeInTheDocument();
  },
};

export const Placeholder: Story = { args: { initial: [] } };

export const Multiple: Story = {
  args: { multiple: true },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);

    await userEvent.click(canvas.getByRole('button', { name: 'Язык истории' }));
    const option = await page.findByRole('option', { name: 'Русский' });

    await userEvent.click(option);
    await expect(option).toHaveAttribute('aria-selected', 'false');
    await userEvent.keyboard('{Escape}');
    await expect(page.queryByRole('listbox')).not.toBeInTheDocument();
  },
};

export const LongList: Story = {
  args: {
    items: Array.from({ length: 30 }, (_, i) => ({
      value: String(i),
      label: `Вариант ${i + 1}`,
    })),
  },
};
