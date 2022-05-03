<script>
  import Button from './Button.svelte';
  import Page from './Page.svelte';

  import { mdiAlertCircleOutline } from '@mdi/js';

  let videoSource = null;
  let loading = false;
  const obtenerVideoCamara = async () => {
    try {
      loading = true;
      const stream = await navigator.mediaDevices.getUserMedia({
        video: true,
      });
      videoSource.srcObject = stream;
      videoSource.play();
      loading = false;
    } catch (error) {
      console.log(error);
    }
  };

  obtenerVideoCamara();
</script>

<header>
  <Page backUrl="/" title="Map scanning">
    {#if loading}
      loading...
    {/if}
    <!-- svelte-ignore a11y-media-has-caption -->
    <video bind:this={videoSource} />

    <div slot="footer">
      <Button icon={mdiAlertCircleOutline}>
        <span>No keystones found</span>
        <span slot="content">
          To recognize the level captureEvents, we need to have 3 markers
          visible
        </span>
      </Button>
    </div>
  </Page>
</header>

<style>
  video {
    height: inherit;
    width: 100%;
    object-fit: cover;
  }
</style>
