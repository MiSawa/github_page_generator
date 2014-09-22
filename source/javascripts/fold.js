//= require 'jquery-2.1.0.min'
$(function(){
    $('#main>h3').each(function(){
        $(this).prepend('<tt>+</tt> ');
        $(this).nextUntil('h1,h2,h3').wrapAll("<div class='fold' />");
        $(this).next().toggle(false);
        $(this).on('click', function(){
            $(this).find('tt').text($(this).find('tt').text() == '+' ? '-' : '+')
            $(this).next().toggle();
        });
    });
});
