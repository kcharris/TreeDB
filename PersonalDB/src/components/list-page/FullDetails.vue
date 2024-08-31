<script setup lang="ts">
    import { ref, computed } from 'vue';
    import { invoke } from "@tauri-apps/api/tauri";

    const props = defineProps(['parent', 'parent_tags'])
    const toggle = ref(false)
    const button_icon = computed(() => {return toggle.value == false ? "mdi-arrow-expand" : "mdi-close-circle-outline"})
    // Throughout the app and this page, a ternary that checks for a property as a bool is used. This boolean conversion is comprehensive across: 0, "", null, undefined, etc.
    const description = computed(()=> "Description: " + (props.parent.description ? props.parent.description : ""))

    function toggleTruncate(){
        toggle.value = !toggle.value
    }
    function openDir(){
        invoke("open_file_explorer", {dirAddress: props.parent?.resource_link})
    }

</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ "Name: " + (parent.name ? parent.name : "")}}</div>
        <div class="d-flex">
            <div class="text-md mr-16">{{"Priority: "+ (parent.priority ? parent.priority : "") }}</div>
            <div class="text-md">
                <p>Completed?: 
                    <v-icon v-if="parent?.completed==false">mdi-checkbox-blank-outline</v-icon>
                    <v-icon color="primary" v-else-if="parent?.completed==true">mdi-checkbox-marked</v-icon>
                </p>
            </div>
        </div>
        <div class="d-flex">
            <div class="text-md mr-8">{{ "Resource: "+ (parent.resource ? parent.resource : "")}}</div>
            <div class="text-md mr-8">{{ "Type: " + (parent.resource_type ? parent.resource_type : "")}}</div>
            <div class="text-md">
                <p>Link to resource:
                    <v-btn v-if="parent?.resource_type=='web'" class="text-truncate text-none text-primary text-decoration-underline" density="compact" :href="parent?.resource_link" target="_blank" variant="text">
                        {{ (parent.resource_link ? parent.resource_link : "")}}
                    </v-btn>
                    <v-btn v-else-if="parent?.resource_type=='dir'" class="text-truncate text-none text-primary text-decoration-underline" density="compact" @click="openDir" variant="text">
                        {{ (parent.resource_link ? parent.resource_link : "")}}
                    </v-btn>
                    <p v-else-if="parent?.resource_type == undefined" class="text-truncate">
                        {{ (parent.resource_link ? parent.resource_link : "")}}
                    </p>
                </p>
            </div>
        </div>
        <div class="text-md">{{"Est Time: " + (parent.est_time ? parent.est_time : "" )}} hrs</div>
        <div class="text-md">{{ "Available: "+ (parent.availability ? parent.availability : "")}}</div>
        <div class="d-flex">
            <div class="text-md mr-16">{{ "Start Time: "+ (parent.start_date ? parent.start_date : "")  }}</div>
            <div class="text-md mr-16">{{"End Time: "+ (parent.end_date ? parent.end_date : "")}}</div>
            <div class="text-md">{{ "Tags Used: " + (parent_tags ? parent_tags.join(", ") : "") }}</div>
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