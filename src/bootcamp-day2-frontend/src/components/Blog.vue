<template>
    <div>
        <button @click="loadPosts">refresh</button>
        This is an example blog
        {{ posts }}
        <input v-model="newBlog" type="text">
        <button @click="addPosts">addPosts</button>
    </div>
</template>

<script>
import { bootcamp_day2_backend } from 'declarations/bootcamp-day2-backend/index';

export default {
    data() {
        return {
            posts: [],
            newBlog: ""
        }
    },
    methods: {
        async addPosts() {
            await bootcamp_day2_backend.add_post(this.newBlog);
        },
        async loadPosts() {
            this.posts = await bootcamp_day2_backend.read_posts();
        }
    },
}
</script>