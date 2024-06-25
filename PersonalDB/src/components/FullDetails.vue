<script setup lang="ts">
    import { ref, computed } from 'vue';
    import { invoke } from "@tauri-apps/api/tauri";

    const props = defineProps({
        parent: Object
    })
    const toggle = ref(false)
    const button_icon = computed(() => {return toggle.value == false ? "mdi-arrow-expand" : "mdi-close-circle-outline"})
    const description = computed(() => {return "Description: " + (props.parent?.description != undefined ? props.parent.description : "")})

    function toggleTruncate(){
        toggle.value = !toggle.value
    }
    function openDir(){
        invoke("open_file_explorer", {dirAddress: props.parent?.resource_link})
    }

</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ "Name: " + (parent?.name != undefined ? parent.name : "")}}</div>
        <div class="d-flex">
            <div class="text-md mr-16">{{"Priority: "+ (parent?.priority != undefined ? parent.priority : "") }}</div>
            <div class="text-md">
                <p>Completed?: 
                    <v-icon v-if="parent?.completed==false">mdi-checkbox-blank-outline</v-icon>
                    <v-icon color="primary" v-else-if="parent?.completed==true">mdi-checkbox-marked</v-icon>
                </p>
            </div>
        </div>
        <div class="d-flex">
            <div class="text-md mr-8">{{ "Resource: "+ (parent?.resource != undefined ? parent.resource : "") }}</div>
            <div class="text-md mr-8">{{ "Type: " + (parent?.resource_type != undefined ? parent.resource_type : "") }}</div>
            <div class="text-md">
                <p>Link to resource:
                    <v-btn v-if="parent?.resource_type=='web'" class="text-truncate text-none text-primary text-decoration-underline" density="compact" :href="parent?.resource_link" target="_blank" variant="text">
                        {{(parent?.resource_link != undefined ? parent.resource_link : "") }}
                    </v-btn>
                    <v-btn v-else-if="parent?.resource_type=='dir'" class="text-truncate text-none text-primary text-decoration-underline" density="compact" @click="openDir" variant="text">
                        {{(parent?.resource_link != undefined ? parent.resource_link : "") }}
                    </v-btn>
                    <v-btn v-else-if="parent?.resource_type == undefined" class="text-truncate text-none" density="compact">
                        {{(parent?.resource_link != undefined ? parent.resource_link : "") }}
                    </v-btn>
                </p>
            </div>
        </div>
        <div class="text-md">{{"Est Time: " + (parent?.est_time != undefined ? parent.est_time : "") }} hrs</div>
        <div class="text-md">{{ "Available: "+ (parent?.availability != undefined ? parent.availability : "") }}</div>
        <div class="d-flex">
            <div class="text-md mr-16">{{ "Start Time: "+ (parent?.start_date != undefined ? parent.start_date : "")  }}</div>
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