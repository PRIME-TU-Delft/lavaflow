<script>
  import NavigationButton from './NavigationButton.svelte';
  import { Router, Route } from 'svelte-navigator';
  import MapScanning from './MapScanning.svelte';
</script>

<Router primary={false}>
  <Route path="/">
    <header>
      <div class="img" />
      <div class="img backdrop" />
      <div class="title">
        <h1>PRIME</h1>
        <h2>AR Lavaflow</h2>
      </div>
    </header>

    <main>
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus lacinia
      condimentum gravida. Nullam in diam cursus elit fermentum varius. Nunc ac
      ornare lorem, aliquam ornare elit. Maecenas in magna elementum, dapibus
      dui sit amet, dictum dolor. Sed congue orci ut hendrerit vulputate.
      Maecenas sed pulvinar augue, convallis lobortis lacus. Proin purus quam,
      dignissim sed arcu id, ornare pellentesque nisi. Suspendisse ullamcorper
      magna a mauris laoreet, sed sagittis massa posuere. Pellentesque semper mi
      ut lacus consequat eleifend. Vivamus cursus venenatis nibh, pharetra
      commodo quam elementum et. Etiam tempus mi eu ligula eleifend fringilla.
      Mauris neque odio, faucibus ut felis et, maximus pulvinar tellus.

      <NavigationButton large to="map-scanning">Start flowing</NavigationButton>
      <NavigationButton large to="/" secondary={true}>
        Teacher corner
      </NavigationButton>
    </main>
  </Route>

  <Route path="map-scanning"><MapScanning /></Route>
  <Route path="map-scanned"><h1>'Ello</h1></Route>
  <Route path="house-scanning" />
  <Route path="house-scanned" />
  <Route path="visualize-ar" />
  <Route path="visualize-vr" />
</Router>

<style>
  h1 {
    margin: 0;
    font-size: 2em;
  }

  h2 {
    font-size: 1.5em;
  }
  header {
    position: relative;
    height: 20rem;
    width: 100%;
  }

  main {
    display: block;
    max-width: 30rem;
    margin: 1rem auto;
    padding: 1rem;
  }

  .img {
    position: absolute;
    bottom: 0;
    background: url(/volcano.jpg);
    background-size: cover;
    background-position: center bottom;
    left: 50%;
    transform: translateX(-50%);
    height: 40rem;
    width: 40rem;
    z-index: 0;
    border-radius: 50%;
  }

  .backdrop {
    background: rgba(220, 73, 73, 21%);
    backdrop-filter: blur(1px) saturate(120%);
  }

  .title {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
    text-align: center;
    font-size: 1.5em;
  }
</style>
