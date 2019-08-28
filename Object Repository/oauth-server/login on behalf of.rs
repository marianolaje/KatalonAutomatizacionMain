<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>login on behalf of</name>
   <tag></tag>
   <elementGuidId>9785cb8f-b82c-4455-a19d-bd7fb74bd74d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;${r_username}&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;${r_password}&quot;
    },
    {
      &quot;name&quot;: &quot;onBehalfOf&quot;,
      &quot;value&quot;: &quot;${r_onbehalfof}&quot;
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
   <restUrl>${r_servidor}/oauth-server/oauth/token?grant_type=password&amp;from=rest</restUrl>
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
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2f196851-5483-47e8-9d01-6984a6e1a468</id>
      <masked>false</masked>
      <name>r_username</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>71e70cd7-6242-427b-8b8d-1afef8c3f6ab</id>
      <masked>false</masked>
      <name>r_password</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f3edbd2e-1850-4ed7-be30-f13b6cb8569a</id>
      <masked>false</masked>
      <name>r_onbehalfof</name>
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
