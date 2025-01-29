# {{ domain.name }} (domain code: {{ domain.code }})

{{ domain.description }}


# Components

{% for component in components  | filter(attribute="domain_name", value=domain.name) %}

## [{{ component.name }} (code {{ component.code }})](components/{{component.identifier}}/{{component.name}}.md)

### Description 

{{ component.description }}

### Errors

{% for error in errors | filter(attribute="component", value=component.name) | filter(attribute="domain", value=domain.name) | sort(attribute="code") %}
- [`{{error.identifier}} {{ error.name }}`]({{component.name}}/{{error.name}}.md)
{% endfor %}

{% endfor %}
