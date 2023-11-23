function convert(obj) {
    let alt_text = "";
    if (("description" in obj) && ("credit" in obj)) {
        if (obj.credit !== null && obj.credit !== '') {
            alt_text = obj.description + ' (' + obj.credit + ')';
        } else {
            alt_text = obj.description;
        }
    } else if (("description" in obj)) {
        alt_text = obj.description;
    } else if (("credit" in obj)) {
        alt_text = obj.credit;
    } else {
        alt_text = obj.story_url;
    }

    let go_icon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-caret-up-fill" viewBox="0 0 16 16">
    <path d="m7.247 4.86-4.796 5.481c-.566.647-.106 1.659.753 1.659h9.592a1 1 0 0 0 .753-1.659l-4.796-5.48a1 1 0 0 0-1.506 0z"/>
  </svg>`;

    return `<div class="card shadow-sm">
    <img class="bd-placeholder-img card-img-top" src="${obj.image_url}" title="${alt_text}" alt="${alt_text}" />
    <div class="card-body">
      <div class="d-flex justify-content-between align-items-right">
        &nbsp;
        <a href="${obj.story_url}" target="_blank" type="button" class="btn btn-sm btn-outline-info" data-bs-toggle="popover" data-bs-trigger="hover" data-bs-content="${alt_text}">${go_icon}</a>
      </div>
    </div>
  </div>`;
}

function notice(message, alert_type) {
    return `<div style="padding-top: 2em;">
    <div class="alert alert-${alert_type} d-flex align-items-center" role="alert">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-exclamation-triangle" viewBox="0 0 16 16">
        <path d="M7.938 2.016A.13.13 0 0 1 8.002 2a.13.13 0 0 1 .063.016.146.146 0 0 1 .054.057l6.857 11.667c.036.06.035.124.002.183a.163.163 0 0 1-.054.06.116.116 0 0 1-.066.017H1.146a.115.115 0 0 1-.066-.017.163.163 0 0 1-.054-.06.176.176 0 0 1 .002-.183L7.884 2.073a.147.147 0 0 1 .054-.057zm1.044-.45a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566z"/>
        <path d="M7.002 12a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 5.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995z"/>
      </svg>
      <div>&nbsp; ${message}</div>
    </div>
  </div>`;
}

function more_button() {
    let more_icon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-caret-down-fill" viewBox="0 0 16 16">
    <path d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z"/>
  </svg>`;

    return `<div style="padding-top: 2em;">
    <button id="loader" type="button" class="btn btn-primary btn-lg">${more_icon} more</button>
  </div>`;
}

function reload(ind) {
    $("#action").empty();
    let posted = 0;

    let api = "/api/next/" + ind;
    $.getJSON(api, function (data) {
        $.each(data, function (_, val) {
            $("#photos").append(convert(val));
            posted += 1;
        });

        $('[data-bs-toggle="popover"]').popover();

        if (ind === 0 && posted === 0) {
            $("#action").append(notice('The service is unavailable now. Please try again later.', 'danger'));
        } else if (posted > 0) {
            ind += posted;
            $("#action").append(more_button());
            $("#loader").click(function () {
                reload(ind);
            });
        } else {
            $("#action").append(notice('You are up to date, no more photos right now.', 'info'));
        }
    });
};

$(function () {
    reload(0);
});