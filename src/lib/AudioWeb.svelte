<script>
  let audioUrl = "";
  let audioPlayer;

  async function fetchAudioProxy() {
    const response = await fetch("/audio");
    const blob = await response.blob();
    console.log("blob from web", blob);
    const url = URL.createObjectURL(blob);
    console.log("url from web", url);
    audioUrl = url;
  }

  function playAudio() {
    console.log("playing audio from web...");
    audioPlayer.play();
    console.log("...audio from web played");
  }

  function pauseAudio() {
    audioPlayer.pause();
  }
</script>

<div>
  <button on:click={fetchAudioProxy}>Fetch audio</button>
  <button on:click={playAudio}>Play</button>
  <button on:click={pauseAudio}>Pause</button>
</div>

<audio bind:this={audioPlayer} src={audioUrl} />
<p>Audio url: {audioUrl}</p>
