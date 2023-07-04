<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    var result: String[]

    async function fileStats() {
      result = await invoke('file_stats')
    }
  </script>
  
  <div>
    <div class="flex flex-row justify-between w-1/4 pl-4">
      <button type="button" class="btn btn-sm variant-filled ml-2" on:click="{fileStats}">Test!</button>
      <p>{result}</p>
    </div>
  </div>