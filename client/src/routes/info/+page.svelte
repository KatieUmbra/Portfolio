<script lang="ts">
    import { Claims } from "$lib/backend/schema/user";
    let { data }: { data: { info: Claims } } = $props();
    import { windowTitle } from "$lib/stores/global";

    windowTitle.set("System information");

    const rankMap: Map<number, string> = new Map();
    rankMap.set(0, "Admin");
    rankMap.set(1, "Trusted");
    rankMap.set(2, "Verified");
    rankMap.set(3, "Unverified");

    console.log(data);
</script>

<p class="m-3">Weclome <b>{data.info.username}</b>! :3</p>
<p class="m-3">Your rank is <b>{rankMap.get(data.info.rank)}</b> :O</p>
