var requested = 0;
var posted = 0;

function convert(obj) {
    var alt_text = "";
    var link_text = "";
    if (("description" in obj) && ("credit" in obj)) {
        alt_text = obj.description;
        if (obj.credit !== null && obj.credit !== '') {
            link_text = obj.description + ' (' + obj.credit + ')';
        } else {
            link_text = obj.description;
        }
    } else if (("description" in obj)) {
        alt_text = obj.description;
        link_text = obj.description;
    } else if (("credit" in obj)) {
        alt_text = obj.credit;
        link_text = obj.credit;
    } else {
        link_text = obj.story_url;
    }
    return '<div class="col"><div class="card shadow-sm"><img src="' + obj.image_url + '" alt="' + alt_text + '" /><div class="card-body"><small class="text-body-secondary"><a href="' + obj.story_url + '">' + link_text + '</a></small></div></div></div>';
}

function reload() {
    $("#action").empty();
    let api = "/api/next/" + requested;
    $.getJSON(api, function (data) {
        $.each(data, function (_, val) {
            $("#photos").append(convert(val));
            posted = posted + 1;
        });
    });

    console.log(posted, requested); // TODO: fix this
    console.log(posted > requested);
    if (posted > requested) {
        requested = requested + posted;
        $("#action").append('<a href="#" class="btn btn-primary my-2">More</a>'); // TODO: add handler to call reload()
    }
};


$(function () {
    reload();
});