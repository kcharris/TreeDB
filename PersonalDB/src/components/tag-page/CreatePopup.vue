<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["create"])
    const dialog = ref(false)
    const new_tag_name = ref("")

    function emitCreate(){
        if (new_tag_name.value != ""){
            emit("create", new_tag_name.value)
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog){
            new_tag_name.value = ""
        }
    })
</script>

<template>
    <v-btn @click="dialog=true" class="bg-primary mr-5">Create New Tag</v-btn>
    
    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Tag Creation</v-card-title>
            <v-card-text>Enter a name for the tag to create. If a tag shares the name of an existing tag, nothing will happen.</v-card-text>
            <v-card-item>
                <v-text-field v-model="new_tag_name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Create"
                @click="emitCreate"
                ></v-btn>
                <v-btn
                    class="bg-grey"
                    text="Cancel"
                    @click="dialog=false"
                ></v-btn>
            </v-card-actions>
            
        </v-card>
            
    </v-dialog>
</template>