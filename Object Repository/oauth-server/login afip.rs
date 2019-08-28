<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>login afip</name>
   <tag></tag>
   <elementGuidId>078fe98e-062f-4c48-9678-4a701d9f5fa5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;sign&quot;,
      &quot;value&quot;: &quot;${r_sign}&quot;
    },
    {
      &quot;name&quot;: &quot;token&quot;,
      &quot;value&quot;: &quot;${r_token}&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic a2xrLWNsaWVudDprbGstc2VjcmV0</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${r_servidor}/oauth-server/oauth/token?grant_type=password&amp;from=afip</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_servidor_oauth</defaultValue>
      <description></description>
      <id>11deb0b3-2afa-44b9-acf0-ce52fcbfa71f</id>
      <masked>false</masked>
      <name>r_servidor</name>
   </variables>
   <variables>
      <defaultValue>'klk-client'</defaultValue>
      <description></description>
      <id>aafbc13b-8962-4491-898e-983c17e51a0a</id>
      <masked>false</masked>
      <name>r_http_username</name>
   </variables>
   <variables>
      <defaultValue>'klk-secret'</defaultValue>
      <description></description>
      <id>76ad5704-33dd-44dc-a0cb-02057fc88daf</id>
      <masked>false</masked>
      <name>r_http_password</name>
   </variables>
   <variables>
      <defaultValue>'PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz48c3NvIHZlcnNpb249IjIuMCI+PGlkIGRzdD0iYWZpcC1nb2JjYmEiIGV4cF90aW1lPSIxNTUyMzM1MjY1IiBnZW5fdGltZT0iMTU1MjMzNDk2NSIgc3JjPSJjbj1BdXRoc2VydmVyLCBvdT1ERVNFSU4sIG89QUZJUCwgYz1BUiIgdW5pcXVlX2lkPSIzMDUyOTc5MzUyIi8+PG9wZXJhdGlvbiB0eXBlPSJsb2dpbiIgdmFsdWU9ImdyYW50ZWQiPjxsb2dpbiBhdXRobWV0aG9kPSJwYXNzcGhyYXNlIiBlbnRpdHk9IjMzNjkzNDUwMjM5IiByZWdtZXRob2Q9IjMiIHNlcnZpY2U9ImFmaXAtZ29iY2JhIiB1aWQ9IjIwMjgyNTE2MTI5Ij48cmVsYXRpb25zPjxyZWxhdGlvbiBrZXk9IjIwMjU5MDI4Mzk3IiByZWx0eXBlPSIxMiIvPjxyZWxhdGlvbiBrZXk9IjIwMjgyNTE2MTI5IiByZWx0eXBlPSIxMiIvPjxyZWxhdGlvbiBrZXk9IjI3MjY0ODUzNDY1IiByZWx0eXBlPSIxMiIvPjwvcmVsYXRpb25zPjwvbG9naW4+PC9vcGVyYXRpb24+PC9zc28+'</defaultValue>
      <description></description>
      <id>2f196851-5483-47e8-9d01-6984a6e1a468</id>
      <masked>false</masked>
      <name>r_token</name>
   </variables>
   <variables>
      <defaultValue>'PIFVSC4Fo5BYX3jIDKNLeeZpifOqt4P0G6bzqLGGUmAZ8RkyRwoBD7DqHOAyModmpAd+KUIL6syQY2M6Nq31vBHceunOqf2xFu2zms5gArk9CHQas8IZZinDRzju++Qj5iDQSxZrhZf1wL8Usonj+zLzZDCc5JsVS6IKMybVU8Y='</defaultValue>
      <description></description>
      <id>71e70cd7-6242-427b-8b8d-1afef8c3f6ab</id>
      <masked>false</masked>
      <name>r_sign</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
