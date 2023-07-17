<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let audioUrl = "";
  let audioPlayer;

  async function fetchAudio() {
    function ok(message) {
        console.log('message', message);
        
        const array = Uint8Array.from(message);
        console.log('array', array);

        const blob = new Blob([array], {type: "audio/mpeg"});
        console.log('blob', blob);

        const url = URL.createObjectURL(blob);
        audioUrl = url;
    }

    await invoke("audio")
    .then((message) => ok(message))
    .catch((error) => console.error(error))
  }

  function playAudio() {
    console.log("playing audio...");
    audioPlayer.play();
    console.log("...audio played");
  }

  function pauseAudio() {
    audioPlayer.pause();
  }
</script>

<div>
  <button on:click={fetchAudio}>Fetch audio</button>
  <button on:click={playAudio}>Play</button>
  <button on:click={pauseAudio}>Pause</button>
</div>

<audio bind:this={audioPlayer} src={audioUrl} />
<p>Audio url: {audioUrl}</p>
