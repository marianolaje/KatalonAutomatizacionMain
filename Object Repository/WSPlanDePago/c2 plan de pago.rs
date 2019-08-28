<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>c2 plan de pago</name>
   <tag></tag>
   <elementGuidId>5ccd8bcc-5c8c-4f86-80ff-fbd45b28ba24</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://apptest.rentascordoba.gob.ar/WSRestPSRM/${r_cuit}/obtenerplanpago/${r_plan}?idPlan=${r_idPlan}&amp;access_token=${r_access_token}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_access_token</defaultValue>
      <description></description>
      <id>6257a613-d37c-4ed1-924d-fe0e35ca3f12</id>
      <masked>false</masked>
      <name>r_access_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.g_cuit</defaultValue>
      <description></description>
      <id>007743b7-0749-4a06-96c1-4e9ebe3365f2</id>
      <masked>false</masked>
      <name>r_cuit</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.g_planDePago</defaultValue>
      <description></description>
      <id>8356e105-feed-4033-bda8-e448699d4f5f</id>
      <masked>false</masked>
      <name>r_plan</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.g_idPlan</defaultValue>
      <description></description>
      <id>dfbc0a0b-d0ce-45b6-bad1-df4e7bf4a345</id>
      <masked>false</masked>
      <name>r_idPlan</name>
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
