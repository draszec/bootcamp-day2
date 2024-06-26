<template>
    <div>
        <h2 class="text-blue-600">Blog posts:</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="loadPosts" class="bg-blue-600 rounded-sm text-white p-4">refresh</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="post in posts"
            class="drop-shadow-xl bg-stone-300 p-4">
                <p>{{ post }}</p>
            </div>
        </div>
        <div class="flex justify-center flex-col">
            <input v-model="newBlog" class="border-2 border-blue-600 p-4" type="text">
            <button @click="addPosts" class="bg-blue-600 rounded-sm text-white p-4">addPosts</button>
        </div>
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
    async mounted(){
        this.loadPosts()
    }
}
</script>