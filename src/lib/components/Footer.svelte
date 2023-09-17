<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  const currentYear = new Date().getFullYear();
</script>

<footer class="w-full p-4 flex items-center justify-center">
  <div
    class="text-gray-400 safe-zone h-[70px] text-center flex flex-col items-center justify-center"
  >
    <small
      >{$LL.LAYOUT.FOOTER.COPYRGHT({
        start: 2020,
        end: currentYear,
      })}
    </small>
    <small class="pt-2 space-x-2">
      <code class="detail">
        {__VERSION__}
      </code>
      <code class="detail">
        {__COMMIT_SHA__}
      </code>
      <code class="detail">
        {__BUILD_DATE__}
      </code>
    </small>
    <small class="py-4">
      <a
        class="underline hover:dark:text-gray-400"
        href="https://github.com/EstebanBorai/estebanborai.com"
        target="_blank">{$LL.LAYOUT.FOOTER.SOURCE_CODE()}</a
      >
    </small>
  </div>
</footer>

<style>
  .detail {
    @apply text-xs dark:text-gray-400 p-1 rounded-md dark:bg-dark-background bg-light-background;
  }
</style>
