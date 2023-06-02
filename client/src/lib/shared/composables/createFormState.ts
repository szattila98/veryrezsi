import { writable } from 'svelte/store';

export type BaseFormStates = 'INITIAL' | 'TECHNICAL_ERROR';

export function createFormState<State extends string>(initial?: State) {
	const formState = writable<State>(initial || ('INITIAL' as State));

	function setFormState(state: State) {
		formState.set(state);
	}

	return { formState, setFormState };
}
