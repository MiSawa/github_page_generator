//= require 'jquery-2.1.0.min'
$(function(){
    $('.fold_button:not(.fold_button_processed)').each(function(){
         $(this).addClass('fold_button_processed')
         $(this).prepend('<tt>+</tt> ');
         $(this).nextAll('.fold_begin').first().nextUntil('.fold_end').wrapAll("<div class='fold' />");
         $(this).nextAll('.fold').first().toggle(false);
         $(this).on('click', function(){
             $(this).find('tt').text($(this).find('tt').text() == '+' ? '-' : '+');
             $(this).nextAll('.fold').first().toggle();
         });
    });
});
