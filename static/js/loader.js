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

    let go_icon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-box-arrow-up-right" viewBox="0 0 16 16">
    <path fill-rule="evenodd" d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5"/>
    <path fill-rule="evenodd" d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0z"/>
  </svg>`;

    let info_icon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-badge-cc-fill" viewBox="0 0 16 16">
    <path d="M2 2a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm3.027 4.002c-.83 0-1.319.642-1.319 1.753v.743c0 1.107.48 1.727 1.319 1.727.69 0 1.138-.435 1.186-1.05H7.36v.114c-.057 1.147-1.028 1.938-2.342 1.938-1.613 0-2.518-1.028-2.518-2.729v-.747C2.5 6.051 3.414 5 5.018 5c1.318 0 2.29.813 2.342 2v.11H6.213c-.048-.638-.505-1.108-1.186-1.108zm6.14 0c-.831 0-1.319.642-1.319 1.753v.743c0 1.107.48 1.727 1.318 1.727.69 0 1.139-.435 1.187-1.05H13.5v.114c-.057 1.147-1.028 1.938-2.342 1.938-1.613 0-2.518-1.028-2.518-2.729v-.747c0-1.7.914-2.751 2.518-2.751 1.318 0 2.29.813 2.342 2v.11h-1.147c-.048-.638-.505-1.108-1.187-1.108z"/>
  </svg>`;

    return `<div class="card shadow-sm">
    <img class="bd-placeholder-img card-img-top" src="${obj.image_url}" title="${alt_text}" alt="${alt_text}" />
    <div class="card-body">
      <div class="d-flex justify-content-between align-items-right">
        &nbsp;
        <div class="btn-group">
          <button type="button" class="btn btn-sm btn-outline-info" data-bs-toggle="popover" data-bs-trigger="click" data-bs-content="${alt_text}">${info_icon}</button>
          <a href="${obj.story_url}" target="_blank" type="button" class="btn btn-sm btn-outline-info" data-bs-toggle="popover" data-bs-trigger="hover" data-bs-content="${alt_text}">${go_icon}</a>
        </div>
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