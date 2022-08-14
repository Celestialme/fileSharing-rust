
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
    .start_server{
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
    <p class="progress">{progress}%</p>
    {/if}
    <input type="text" placeholder="Adress" bind:value={value} >
    <button  on:click={openFolder}>choose folder</button>
    <button class="start_server" bind:this={start_button} on:click={start_server}>Start Server</button>
    
</div>

<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { onDestroy, onMount } from 'svelte';
import { open } from '@tauri-apps/api/dialog';
let showProgress = false;
let progress = 0;
let start_button: HTMLButtonElement;
let value = "127.0.0.1:7878";
let selectedFolder: string|string[]|null;
let unlisten: UnlistenFn;
let unlisten2: UnlistenFn;
function start_server(){
    invoke("start_server",{
        folderPath:selectedFolder,
        address:value
    });
}



async function openFolder(){
    selectedFolder = await open({
multiple: false,
  directory: true,
});
 

}











onMount(()=>{

listen('server_status', (e:any) => {
    if(e.payload == true){
        start_button.style.backgroundColor = "green";
        showProgress = true;
    }
}).then(_unlisten=>unlisten=_unlisten)
listen('progress', (e:any) => {
   
        progress=e.payload;
  
}).then(_unlisten=>unlisten2=_unlisten)

});
onDestroy(()=>{
  unlisten?.();
  unlisten2?.();
});

</script>

