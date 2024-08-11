<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["restore"])
    const props = defineProps(['backup_name'])
    const dialog = ref(false)
    const restore_name = ref("")

    function emitRestore(){
        if (restore_name.value != ""){
            emit('restore', {backupName: props.backup_name, newName: restore_name.value})
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog.value == true){
            restore_name.value = ""
        }
    })

</script>

<template>
    <v-btn @click="dialog=true" color="warning" icon="mdi-database-cog"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Restore Backup</v-card-title>
            <v-card-subtitle>{{ props.backup_name }}</v-card-subtitle>
            <v-card-text>Choose a new name for the backup selected to restore it to the DB Management Page. If the new database shares the name with an existing database on the main page, nothing will happen.</v-card-text>
            <v-card-item>
                <v-list>
                    <v-text-field v-model="restore_name"></v-text-field>
                </v-list>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Restore"
                @click="emitRestore"
                ></v-btn>
                <v-btn
                    class="bg-grey"
                    text="Back"
                    @click="dialog=false"
                ></v-btn>
            </v-card-actions>
            
        </v-card>
    </v-dialog>
</template>