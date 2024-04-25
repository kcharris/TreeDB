<script setup lang="ts">
    defineProps({
        parent: Object
    })

    function getText(prefix, parent_val){
        return parent_val == "" || parent_val == NaN || parent_val == undefined || parent_val == null ? prefix + ": " : prefix + ": " + parent_val
    }
</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ getText("Name", parent.name)}}</div>
        <div class="d-flex">
            <div class="text-md mr-5">{{getText("Priority", parent.priority) }}</div>
            <div>               </div>
            <div class="text-md">{{ getText("Completed?", parent.completed)}}</div>
        </div>
        <div class="text-md">{{ getText("Link to resource", parent.resource) }}</div>
        <div class="text-md">{{getText("Est Time", parent.est_time)}} hrs</div>
        <div class="text-md">{{ getText("Availability", parent.availability) }}</div>
        <div class="d-flex">
            <div class="text-md mr-5">{{ getText("Start Time", parent.start_date) }}</div>
            <div>               </div>
            <div class="text-md">{{getText("End Time", parent.end_date) }}</div>
        </div>
        <div class="text-md text-truncate text-nowrap"> {{getText("Description", parent.description) }}</div>
    </div>
</template>