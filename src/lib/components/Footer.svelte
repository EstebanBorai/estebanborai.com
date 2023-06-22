<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  const currentYear = new Date().getFullYear();
</script>

<footer class="w-full px-4 flex items-center justify-center">
  <div class="safe-zone h-[70px] flex items-center justify-center">
    <small
      >{$LL.LAYOUT.FOOTER.COPYRGHT({
        start: 2020,
        end: currentYear,
      })}</small
    >
  </div>
</footer>
