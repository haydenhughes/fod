// Handle modals

$(document).ready(function() {
  $(".modal-button")
    .add(".modal-background")
    .add(".modal-close")
    .add(".modal-card-head > .delete")
    .click(function() {
      $("#" + $(this).data("target")).toggleClass("is-active");
  });
});
