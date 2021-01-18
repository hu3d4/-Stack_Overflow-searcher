// function add() {
//     let form = document.getElementById(searchButton);
//     console.log(form);
// }

function hoge(a, b) {
    console.log('hoge!');
    return [a, b];
}

function afterHoge() {
    console.log('after hoge!');
}

// hogeが実行された後にafterHogeを実行する関数を生成
var newHoge = callAfter(hoge, afterHoge);

console.log(newHoge('1', '2'));
// 出力
// > hoge!
// > after hoge!
// > [ '1', '2' ]

function add() {
    // document.getElementById(customSearch);
    document.forms['searchForm'].submit();
}

// let google = function (y) {
//     let
// }

function addHistory() {
    document.forms['addHistory'].submit();
}

function customSearch() {
    document.forms['customSearch'].submit();
}



// acc.forEach(function(elem, index) {
//     return (index)
// })
// return [addHistory(), customSearch()]


function addHistory() {
    document.forms['addHistory'].submit();
}

function customSearch() {
    document.forms['customSearch'].submit();
}

function callAfter() {
    let acc = [addHistory(), customSearch()]
    while (acc.length > 0) {
        acc.shift();
    }
}