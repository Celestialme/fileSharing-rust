<style>
   @import "@static/global.css";
    body {
      display:flex;
      flex-direction: column;
      background-color:gray;
        height: 100vh;
        width: 100vw;
        justify-content: center;
        align-items: center;
    }
    .top-panel, .bottom-panel {
        padding: 5px;
        margin-bottom:auto;
        color:white;
        
    }
   .top-panel .controlls{
        border:1px solid white;
        font-size: 25px;
        padding:5px;
        width: 100px;
        text-align: center;
        margin:-2px;
    }
    .top-panel .controlls:first-of-type{
        border-top-left-radius: 6px;
        border-bottom-left-radius: 6px;
    }
    .top-panel .controlls:last-of-type{
        border-top-right-radius: 6px;
        border-bottom-right-radius: 6px;
    }
    .controlls{
        display:inline-block
    }
    input:checked + .controlls{
        background-color:#09dfd5;
    }
    .controlls:hover{
        background-color:darkgrey;
    }

</style>

<body>
    <div class="top-panel">
            
                 <input bind:group={comp} value={Send} type="radio" id="send" name="radio" checked hidden>
                 <label class="controlls"  for="send">Send</label>
             
          
                <input bind:group={comp} value={Receive} type="radio" id="receive" name="radio"  hidden>
                <label  class="controlls" for="receive">Receive</label>
       
        


    </div>

    <div class="bottom-panel">
        <svelte:component this={comp}/>
    </div>

</body>
<script lang="ts">
    let comp = Send;
    import Send from "@src/components/Send.svelte";
    import Receive from "@src/components/Receive.svelte";
</script>

