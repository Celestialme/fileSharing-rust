
<style>
    input{
        margin-bottom:10px;
        text-align: center;
        padding:5px;
        border-radius: 6px;
        border: none;
        outline: none;
    }
    input,button{
        font-size: 25px;
       
    }
    button{
        background-color:white;
        color:black;
        display: block;
        padding:10px 10px;
        width:auto;
        margin:auto;
        border-radius: 6px;
        border:1px solid gray
    }
    button:hover{
        background-color:#80e1f9;
        color: white;
    }
    .disabled:hover{
        background-color:white;
        color:black;
    }
    .send_folder{
        position: absolute;
        bottom:0;
        left:50%;
        transform: translateX(-50%);
    }
    .progress{
        text-align: center;
        font-size: 20px;
        margin-bottom: 10px;
    }
</style>
<div >
    {#if showProgress}
    <p class="progress">{Math.floor(sent_size /folder_size * 100)}%</p>
    {/if}
    <input type="text" placeholder="Adress" bind:value={value} >

    
    <button on:click={openFolder}>Select Folder</button>
    <button class="send_folder" class:disabled={!selectedFolder} on:click={start_sending}>Send Folder</button>
</div>

<script lang="ts">
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { onDestroy, onMount } from 'svelte';
let showProgress = false;
let value = "127.0.0.1:7878";
let selectedFolder: string|string[]|null;
let folder_size=0;
let sent_size=0;
async function openFolder(){
    selectedFolder = await open({
multiple: false,
  directory: true,
});
 folder_size =  await invoke("calculate_size",{path:selectedFolder});

}

 function start_sending(){
    sent_size=0;
    if(!selectedFolder) return

    showProgress =true;
    invoke("start_sending",{path:selectedFolder,address:value,totalSize:folder_size}).then(_=>alert('finished'));
}
let unlisten: UnlistenFn;

onMount(()=>{

  listen('progress', (e:any) => {
   sent_size += e.payload
}).then(_unlisten=>unlisten=_unlisten)

});
onDestroy(()=>{
    unlisten?.();
});

</script>

