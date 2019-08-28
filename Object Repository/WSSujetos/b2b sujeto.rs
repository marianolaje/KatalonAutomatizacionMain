<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>b2b sujeto</name>
   <tag></tag>
   <elementGuidId>81a5f62e-1aca-4b84-93af-b2ee2d6ddbee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${r_servidor}/WSRestSujetos/InformacionSujetos/sujeto/cuit/${r_cuit}?access_token=${r_access_token}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a6cee5dc-06cf-412b-bb0c-8550b960d67e</id>
      <masked>false</masked>
      <name>r_cuit</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2d2f2969-70d0-4bf6-ae46-d3d515e4027c</id>
      <masked>false</masked>
      <name>r_servidor</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>58fc8ae9-7c84-46a1-ad53-a0e629ec9adc</id>
      <masked>false</masked>
      <name>r_access_token</name>
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
