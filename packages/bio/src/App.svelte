<script lang="ts">
	import Tailwind from "./Tailwind.svelte";
	export let name: string;
</script>

<Tailwind />
<main>
	<h1>Welcome {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	
</style>
