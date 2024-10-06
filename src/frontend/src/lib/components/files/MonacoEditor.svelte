<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	export let contents: string;
	export let type: string = 'text/plain';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;

	onMount(async () => {
		// Import our 'monaco.ts' file here
		// (onMount() will only be executed in the browser, which is what we want)
		monaco = (await import('./monaco')).default;

		// Your monaco instance is ready, let's display some code!
		const editor = monaco.editor.create(editorContainer, {
			value: contents,
			language: type,
			theme: 'vs-dark',
			hover: {
				enabled: false
			},
			suggest: {
				showIcons: false,
				showStatusBar: false,
				showWords: false,
				showMethods: false,
				showFunctions: false,
				showConstructors: false,
				showFields: false,
				showVariables: false,
				showClasses: false,
				showStructs: false,
				showInterfaces: false,
				showModules: false,
				showProperties: false,
				showEvents: false,
				showOperators: false,
				showUnits: false,
				showValues: false,
				showConstants: false,
				showEnums: false,
				showEnumMembers: false,
				showKeywords: false,
				showColors: false,
				showFiles: false,
				showReferences: false,
				showFolders: false,
				showTypeParameters: false,
				showSnippets: false
			},
			minimap: {
				enabled: false
			},
			lightbulb: {
				enabled: false
			},
			parameterHints: {
				enabled: false
			},
			quickSuggestions: {
				other: false,
				comments: false,
				strings: false
			}
		});
		const model = monaco.editor.createModel(contents, type);
		editor.setModel(model);

		editor.onDidChangeModelContent(() => {
			contents = editor.getValue();
		});
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});
</script>

<div>
	<div class="editor" bind:this={editorContainer} />
</div>

<style>
	.editor {
		width: 100%;
		height: 600px;
	}
</style>
