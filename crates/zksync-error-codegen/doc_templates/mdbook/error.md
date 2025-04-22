# {{error.identifier }} {{ error.name }} 


- **Domain**: {{ error.domain }}
- **Component**: {{ error.component }}
- **Error Code**: {{ error.code }}
- **Message**: `{{ error.identifier }} {{ error.message }}`

{% if error.documentation.short_description %}
## Short description 
{{ error.documentation.short_description }}
{% endif %}

{% if error.fields | length > 0 %}
## Fields

{% for field in error.fields %}
- `{{ field.name }} : {{ field.type }}`

{% endfor %}

{% endif %}


{% if error.documentation %}
## Description 

{{ error.documentation.description }}

{% if error.documentation.likely_causes | length > 0 %}
##    Likely Causes
    {% for cause in error.documentation.likely_causes %}
- {{ cause.cause }}
{% if cause.owner %}
   - **Owner**: {{ cause.owner.name }} (Version {{ cause.owner.version }})
{% endif %}

{% if cause.report %}
   - **Report to**: {{ cause.report }}
{% endif %}

{% if cause.references | length > 0 %}
   - **References**
{% for reference in cause.references %}
     -{{ reference }}
{% endfor %}

{% for fix in cause.fixes %}
  - **Possible fix**: {{ fix }}
{% endfor %}

{% endif %}
{% endfor %}
{% endif %}
{% endif %}




## Language Bindings 

| Language   | Type                            |
|:----------:|:-------------------------------:|
{% for name, binding in error.bindings %} | {{ name }} | {{ binding.expression }} | {% endfor %}
