<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["rename"])
    const props = defineProps(['backup_name'])
    const dialog = ref(false)
    const new_name = ref("")

    function emitRename(){
        if (new_name.value != ""){
            emit("rename", {backupName: props.backup_name, newName: new_name.value})
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog){
            new_name.value = ""
        }
    })

</script>

<template>
    <v-btn @click="dialog=true" color="indigo-lighten-1" icon="mdi-square-edit-outline"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Rename Backup</v-card-title>
            <v-card-subtitle >{{ props.backup_name }}</v-card-subtitle>
            <v-card-text>Enter a different name for the backup. If the new name shares the name of an existing backup, nothing will happen.</v-card-text>
            <v-card-item>
                <v-text-field v-model="new_name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Rename"
                @click="emitRename"
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