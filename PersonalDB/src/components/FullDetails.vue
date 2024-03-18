<script>
    export default{
        data: () =>({
            item_name: "cat"
        })
    }
</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ "Name: " + item_name }}</div>
        <div class="d-flex">
            <div class="text-md mr-5">Priority: 100</div>
            <div>               </div>
            <div class="text-md">Completed?: False</div>
        </div>
        <div class="text-md">Link to resource: link</div>
        <div class="text-md">Est Time: 3.5h</div>
        <div class="d-flex">
            <div class="text-md mr-5">Start Time: Date from datepicker</div>
            <div>               </div>
            <div class="text-md">End Time: Date from datepicker</div>
        </div>
        <div class="text-md text-truncate text-nowrap">Description: limit to one line that can expand limit to one line that can expandlimit to one line that can expandlimit to one line that can expand</div>
    </div>
</template>