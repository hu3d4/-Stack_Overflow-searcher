function addHistory() {
    document.forms['addHistory'].submit();
}

function customSearch() {
    let input_text = document.getElementById("input").value;
    document.forms['customSearch'].elements['q'].value = input_text;
    document.forms['customSearch'].submit();
}

function callAfter() {
    let acc = [customSearch(), addHistory()]
    while (acc.length > 0) {
        acc.shift();
    }
}
