function searchGoogle(q) {
    document.forms['searchForm'].elements['q'].value = q;
    document.forms['searchForm'].submit();
    return false;
}

function selectId() {
    document.querySelector(input).value;
}
