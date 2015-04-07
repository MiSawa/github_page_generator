
class SecretPageExtension < Middleman::Extension
  #option :enable_secret, true, 'Enable secret mode'
  #option :secret_link, '/secret/{path}', 'Path secret pages are generated at.'
  option :secret_tag, 'secret', 'Frontmatter tag for secret pages.'
  def manipulate_resource_list(resources)
    #return resources.reject do |resource|
    #  resource.data[options.secret_tag]
    #end
    for resource in resources
      @app.ignore resource.path if resource.data[options.secret_tag]
    end
    return resources
    #return resources.reject do |resource|
    #  resource.data[options.secret_tag]
    #end
    #resources.map do |resource|
    #  if resource.data[options.secret_tag]
    #    @app.ignore resource.path
    #    #'secret/' + p.destination_path
    #    resource.destination_path = Middleman::Blog::UriTemplates::apply_uri_template(
    #      Middleman::Blog::UriTemplates::uri_template(options.secret_link),
    #      path: resource.destination_path
    #    )
    #  end
    #  resource
    #end
  end
end

::Middleman::Extensions.register(:secret, SecretPageExtension)

