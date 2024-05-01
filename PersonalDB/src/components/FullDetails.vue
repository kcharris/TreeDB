<script setup lang="ts">
    defineProps({
        parent: Object
    })

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
        <div class="text-md text-truncate text-nowrap"> {{"Description: "+ (parent?.description != undefined ? parent.description : "") }}</div>
    </div>
</template>