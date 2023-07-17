<script>
  import AudioWeb from "./lib/AudioWeb.svelte";
  import AudioBackend from "./lib/AudioBackend.svelte";
</script>

<main class="container">
  <h1>Audio reader</h1>

  <div>
    <h2>Audio fetched from web</h2>
    <AudioWeb />
    <br />
    <h2>Audio fetched from back-end</h2>
    <AudioBackend />
  </div>
</main>
