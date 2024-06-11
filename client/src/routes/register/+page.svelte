<script lang="ts">
    import { goto } from '$app/navigation';
    import { redirect } from '@sveltejs/kit';
import { createForm } from 'felte';

const {form} = createForm({});

function handleSuccess(event) {
    const { response, ...context } = event.detail;
    goto("/");
}
</script>

<div class="grid gird-cols-1 gap-2 bg-pink-200 w-full min-h-screen items-center justify-center">
    <form use:form action="/register" method="POST" on:feltesuccess="{handleSuccess}">
        <p class="m-3 text-xl font-bold">Register</p>
        <input type="text" class="input input-bordered w-full max-w-ws m-3" name="username" placeholder="Username"/>
        <input type="text" class="input input-bordered w-full max-w-ws m-3" name="displayName" placeholder="Display Name"/>
        <input type="email" class="input input-bordered w-full max-w-ws m-3" name="email" placeholder="Email"/>
        <input type="password" class="input input-bordered w-full max-w-ws m-3" name="password" placeholder="Password"/>
        <button type="submit" class="btn m-3">Submit</button>
    </form>
</div>
