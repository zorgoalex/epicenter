import type { KeyboardEventPossibleKey, KeyboardEventSupportedKey } from '.';

const CODE_TO_LAYOUT_INDEPENDENT_KEY_MAP: Partial<Record<string, string>> = {
	Backquote: '`',
	Minus: '-',
	Equal: '=',
	BracketLeft: '[',
	BracketRight: ']',
	Backslash: '\\',
	Semicolon: ';',
	Quote: "'",
	Comma: ',',
	Period: '.',
	Slash: '/',
};

export function normalizeKeyboardEventKey(
	event: KeyboardEvent,
): KeyboardEventPossibleKey | KeyboardEventSupportedKey {
	if (event.code === 'ControlRight') {
		return 'rightcontrol';
	}

	const layoutIndependentKey = CODE_TO_LAYOUT_INDEPENDENT_KEY_MAP[event.code];
	if (layoutIndependentKey) {
		return layoutIndependentKey as KeyboardEventPossibleKey;
	}

	return event.key.toLowerCase() as KeyboardEventPossibleKey;
}
