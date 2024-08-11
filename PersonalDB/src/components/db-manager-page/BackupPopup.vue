<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["backup"])
    const props = defineProps(['db_name'])
    const dialog = ref(false)
    const backup_name = ref("")

    function emitBackup(){
        if (backup_name.value != ""){
            emit('backup', {dbName: props.db_name, backupName: backup_name.value})
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog){
            let curr_date = new Date()
            let date_str:String = curr_date.toISOString().slice(0,19);
            date_str = date_str.replace(":", "")
            date_str = date_str.replace(":", "")

            backup_name.value = props.db_name + date_str
        }
    })

</script>

<template>
    <v-btn @click="dialog=true" color="warning" icon="mdi-database-cog"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="700"
        >
        <v-card>
            <v-card-title>Create Backup</v-card-title>
            <v-card-text>Use the default name or create a unique name for the backup to be created. Backups can be found and used from the backup manager page found at the left nav bar.
                <br><br>
                The default name includes an ISO formated date for convenience.
            </v-card-text>
            <v-card-item>
                <v-text-field v-model="backup_name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Backup"
                @click="emitBackup"
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