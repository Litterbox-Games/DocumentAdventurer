<script>
    import Greet from '../lib/Greet.svelte'
    import { LightSwitch } from '@skeletonlabs/skeleton';
</script>

<span class="flex justify-between items-center p-4">
    <h1 class="bg-gradient-to-br text-5xl from-blue-500 to-cyan-300 bg-clip-text text-transparent box-decoration-clone">
        Welcome to SvelteKit
    </h1>
    <LightSwitch />
</span>
<Greet />