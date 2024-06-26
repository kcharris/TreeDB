<script setup lang="ts">
import { ref, watch} from "vue";

    const prop = defineProps(["address_str", "address_type"])
    const emit = defineEmits(["addAddress"])
    const dialog = defineModel({default:false})
    const address = ref("")
    const address_type = ref("")
    const items = [
        undefined,
        "web",
        "dir"
    ]

    function emitAddAddress(){
        emit("addAddress", [address_type.value, address.value])
        dialog.value = false
    }

    // address is never cleared, only updated
    watch(dialog, (x) => {
        if (x == true){
            address.value = prop.address_str
            address_type.value = prop.address_type
        }
    })
</script>

<template>
    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
        <template v-slot:actions>
            <v-card-text class="ms-auto">
                <v-row>
                    <p class="my-auto">Enter the full address of the resource and select the target.</p>
                </v-row>
                <v-row>
                    <v-text-field
                        v-model="address"
                        maxlength="200"
                        counter
                        clearable
                    >
                    </v-text-field>
                </v-row>
                <v-row>
                    <v-select
                        v-model="address_type"
                        :items="items"
                        label="Select Target"
                        class="ms-auto"
                    >
                    </v-select>
                    
                    <v-btn
                        class="ml-5 mr-2 bg-primary"
                        text="Update"
                        @click="emitAddAddress"
                    ></v-btn>
                    <v-btn
                        class="ms-auto bg-grey"
                        text="Back"
                        @click="dialog=false"
                    ></v-btn>
                </v-row>
            </v-card-text>
            
        </template>  
        </v-card>
            
    </v-dialog>
</template>
