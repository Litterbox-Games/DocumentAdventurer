<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let name = ''
    let greetMsg = ''
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }
  </script>
  
  <div>
    <div class="flex flex-row justify-between w-1/4 pl-4">
      <input type="text" class="input pl-2" id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
      <button type="button" class="btn btn-sm variant-filled ml-2" on:click="{greet}">Greet</button>
    </div>
    <div class="flex justify-center m-10">
      <div class="w-screen">
        <h1 class="bg-gradient-to-br text-3xl from-red-500 to-yellow-500 bg-clip-text text-transparent box-decoration-clone text-center">{greetMsg}</h1>
      </div>
    </div>
  </div>