<script setup lang="ts">
    import { ref, computed } from 'vue';
    const props = defineProps({
        parent: Object
    })
    const toggle = ref(false)
    // const description_class = computed(() => {return `text-md ${toggle.value == false ? "text-truncate" : ""}`})
    const button_icon = computed(() => {return toggle.value == false ? "mdi-arrow-expand" : "mdi-close-circle-outline"})
    const description = computed(() => {return "Description: " + (props.parent?.description != undefined ? props.parent.description : "")})

    function toggleTruncate(){
        toggle.value = !toggle.value
    }

</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ "Name: " + (parent?.name != undefined ? parent.name : "")}}</div>
        <div class="d-flex">
            <div class="text-md mr-5">{{"Priority: "+ (parent?.priority != undefined ? parent.priority : "") }}</div>
            <div>               </div>
            <div class="text-md">{{ "Completed?: "+ (parent?.completed != undefined ? parent.completed : "") }}</div>
        </div>
        <div class="text-md">{{ "Link to resource: "+ (parent?.resource != undefined ? parent.resource : "") }}</div>
        <div class="text-md">{{"Est Time: " + (parent?.est_time != undefined ? parent.est_time : "") }} hrs</div>
        <div class="text-md">{{ "Available: "+ (parent?.availability != undefined ? parent.availability : "") }}</div>
        <div class="d-flex">
            <div class="text-md mr-5">{{ "Start Time: "+ (parent?.start_date != undefined ? parent.start_date : "")  }}</div>
            <div>               </div>
            <div class="text-md">{{"End Time: "+ (parent?.end_date != undefined ? parent.end_date : "") }}</div>
        </div>
        <div>
            <v-textarea
                class="text-md"
                :prepend-icon="button_icon"
                @click:prepend="toggleTruncate"
                rows="1"
                :max-rows="toggle == false ? 1 : 50"
                row-height="15"
                auto-grow
                variant="underlined"
                readonly
                :model-value="description"
                >   
            </v-textarea>
        </div>
    </div>
</template>