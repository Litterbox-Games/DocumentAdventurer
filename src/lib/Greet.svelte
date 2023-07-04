<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { FileButton } from '@skeletonlabs/skeleton';

    let name = ''
    let greetMsg = ''
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }
  </script>
  
  <div>
    <span>
      <input class="input" type="text" id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
      <button type="button" class="btn variant-filled" on:click="{greet}">Greet</button>
    </span>
    <p>{greetMsg}</p>
    <FileButton name="file"/>
  </div>