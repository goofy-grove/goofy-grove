# Chat previews

Run `npm run storybook`, then open `Chat/Conversation`, `Chat/Messages`, `Chat/Composer` or `Chat/List item`.

## Chat components

Chat UI belongs to the `pages/chats` slice. `ChatView` is exported through
`@pages/chats`; its internal components live in `ui/chat-view/components`, each with
its own props and styles. The formatting toolbar is nested in `rich-editor/components`.
Rich-text styles live beside `rich-message` and are imported by the viewer and editor. The role badge has its own shared stylesheet; other styles belong to their components.

Chat data types and constants live in `model`; the shared TipTap schema and content conversion live in `lib`. Components use `useTranslation` with the `chat` key prefix. Stories can import internal components directly
for isolated previews without exposing them in the slice's public API.

- `ChatView`: controlled composition of users, connected Characters, message timeline and composer.
- `CharacterPanel`: request a specific Character reply; offer editing only when `ownerId === currentUserId`.
- `ChatComposer`: selects from the current user's Personas (with avatar inside the selector). These are not chat participants. API messages use the author returned by the server; absent authors render a localized fallback.
- `ChatMessage`, `RichEditor`, `RichMessage`: the same TipTap schema for composing and reading JSON; no attachments.
- `VirtualTimeline`: TanStack Virtual, measured variable-height rows, stable message IDs, default page size `CHAT_PAGE_SIZE = 50`. Calls `loadOlder(pageSize)` near the top. The parent prepends older messages in chronological order and updates `hasOlder`. Loading failures expose retry. End anchoring preserves the visible message on prepend and follows replies only when the reader was at the bottom. Active message editors remain mounted while scrolling.

`ChatView` accepts current-user Personas, chat users and connected Characters as separate inputs. Sending, triggering a Character, editing a Character and loading history are callbacks. Backend permissions must enforce ownership too; the UI only controls visible actions. The application chat route is not connected yet.

## Story scenarios

- Conversation, first message, generating, failed message and read-only.
- `PagedHistory`: 250 local messages; starts with the most recent 50 and loads older pages of 50 with a delay.
- `HistoryLoadError`: first page request fails; retry succeeds.
- `CustomBackground` / `MobileWithBackground`: a local SVG forest background and readable message surfaces. The `backgroundUrl` accepts another image URL.
- The Character panel has one owned and one other-user Character. Both can be requested to reply; only the owned Character has editing. Editing opens the existing character form in a local Storybook dialog.
- Mobile: compact composer, 44px controls, selection-based TipTap BubbleMenu and an `Aa` toggle for full formatting. The chat responds to visual viewport resizing when the phone keyboard opens.
- RU/EN copy uses the application's locale resources; Russian labels are «Персона» and «Персонаж».

Fixtures, timers and demo editing stay in this directory. Sending, retry and Character responses are local simulations, not API/model calls. Reloading a story resets the scene.

Interaction stories cover formatted sending, Persona selection, ownership of character editing, requesting a specific Character and page loading. They execute in the browser; type checks and builds alone do not run them.

References: [TanStack chat anchoring](https://tanstack.com/virtual/latest/docs/api/virtualizer), [TipTap BubbleMenu](https://tiptap.dev/docs/editor/extensions/functionality/bubble-menu).

## Chat list item

`pages/chats/ui/chat-list-item` contains the controlled list card with scene artwork,
message preview, character count, unread count and generation state. `onOpen` and
optional `onActions` receive the chat UID; routing and menus belong to the caller.
Stories cover read/unread, generation, empty history, missing/broken artwork, long
content, a 320px viewport, English labels, and a list of several scenes.

## Server contract and presentation adapters

The server's Rust response types are the source of truth. Client DTOs preserve
`uid`, `creator_uid`, `avatar_uid`, `created_at`, nullable message authors/replies,
`is_removed`, and the `{ messages, next_page }` pagination envelope. Date strings are converted to `Date` by the client schemas. The client
schemas live in `shared/api/chats` and `shared/api/messages`; no message requests
or application route integration are added by this refactor.

`pages/chats/model` maps DTOs to presentation data. Ownership comes from
`creator_uid` and the current user; owner labels are supplied explicitly from
known users. File identifiers stay intact and `FileAvatar` resolves images.
`previewUrl` is reserved for local previews. `ChatPersona` restricts the composer
inputs; delivery failures, edited labels, and simulated generation are demo/UI
state, not additional fields expected from the server.

Message `content` remains a string on the wire. The frontend serializes TipTap
JSON into that string and reads it using the same schema as the editor. Ordinary
text, malformed JSON, and unsupported documents render as literal text with
line breaks, never as raw HTML. Removed content is replaced with a localized
notice. This encoding is a frontend convention; no server format changes are
required.

`lastMessage` remains an optional presentation prop for list previews. It is not
part of the current chat DTO; its server implementation and live integration are
deferred. No extra requests derive a last-message preview.

Existing interaction stories are retained but were not executed for this
refactor. No tests, static checks, builds, or browser verification were run.
