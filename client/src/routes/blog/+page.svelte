<script lang="ts">
import DOMPurify from "isomorphic-dompurify";
import { Carta, MarkdownEditor } from "carta-md";

const carta = new Carta({
    sanitizer: DOMPurify.sanitize,
    theme: "github-light"
});

let value = '';
</script>

<MarkdownEditor {carta} bind:value />
