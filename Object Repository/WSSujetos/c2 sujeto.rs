<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>c2 sujeto</name>
   <tag></tag>
   <elementGuidId>8b4d33e4-9138-4e30-8a81-d4e38415422a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${r_servidor}/WSRestSujetos/InformacionSujetos/Sujeto/${r_cuit}?access_token=${r_access_token}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'20256103584'</defaultValue>
      <description></description>
      <id>9f7ed6d6-f684-42a6-98fc-f89cf5295724</id>
      <masked>false</masked>
      <name>r_cuit</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.g_servidor_crm</defaultValue>
      <description></description>
      <id>3428ffd9-a048-4241-ae81-7d52ce409f13</id>
      <masked>false</masked>
      <name>r_servidor</name>
   </variables>
   <variables>
      <defaultValue>'992f557a-62e2-48a9-9745-2f36b752a9db'</defaultValue>
      <description></description>
      <id>a8a38d15-4198-4a33-a2eb-122aa9c81836</id>
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
