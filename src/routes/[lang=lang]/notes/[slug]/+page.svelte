<script lang="ts">
  export let data;

  const avatarUrl = '/images/whoami.jpg';
  const yoe = new Date().getFullYear() - 2016;
</script>

<div id="note-container">
  <svelte:component this={data.content} />
</div>
<div
  class="group grid grid-cols-[100px,auto] gap-4 border-t border-gray-400 pt-4"
>
  <figure
    class="grayscale group-hover:grayscale-0 rounded-full overflow-hidden flex justify-center items-center h-[100px] w-[100px]"
  >
    <img src={avatarUrl} alt="whoami" height="100" width="100" />
  </figure>
  <article>
    <h1 class="text-lg">Esteban Borai</h1>
    <div class="text-sm space-y-2">
      <p>
        Hi there! I'm a <u>Rust Software Engineer</u> with {yoe} years of experience
        in Systems and Web Programming using Rust &amp; TypeScript.
      </p>
      <p>
        I'm passionate about Open-Source and enjoy reading books, working out
        &amp; playing videogames.
      </p>
      <p>
        I've had the opportunity to work with companies like InfinyOn, GOintegro
        &amp; Teleperformance.
      </p>
    </div>
  </article>
</div>
