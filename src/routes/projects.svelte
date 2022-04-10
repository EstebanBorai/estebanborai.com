<script lang="ts">
  import Project, {
    Language,
  } from '$lib/modules/projects/components/Project.svelte';
</script>

<ul class="my-4 flex flex-col md:grid md:grid-cols-3 md:gap-6">
  <Project
    name="http-server"
    description="Simple and configurable command-line HTTP server"
    image="/images/svg/http-server.svg"
    language={Language.Rust}
    repository="https://github.com/EstebanBorai/http-server"
  />
  <Project
    name="local-ip-address"
    description="Retrieve system's local IP address and Network Interfaces/Adapters on Linux, macOS and Windows"
    language={Language.Rust}
    image="/images/svg/http-server.svg"
    repository="https://github.com/EstebanBorai/local-ip-address"
  />
  <Project
    name="http-auth-basic"
    description="HTTP Basic Authentication Scheme (RFC 7617 and RFC 2617 compilant, base64-encoded credentials) for Rust applications"
    language={Language.Rust}
    image="/images/svg/http-server.svg"
    repository="https://github.com/EstebanBorai/http-auth-basic"
  />
  <Project
    name="manzana"
    description="Svelte form utility library"
    language={Language.TypeScript}
    image="/images/svg/http-server.svg"
    repository="https://github.com/EstebanBorai/manzana"
  />
</ul>
